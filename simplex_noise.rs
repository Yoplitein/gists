// Port of https://github.com/KdotJPG/OpenSimplex2/blob/3c64be93f7fa/java/OpenSimplex2F.java

/**
 * K.jpg's OpenSimplex 2, faster variant
 *
 * - 2D is standard simplex implemented using a lookup table.
 * - 3D is "Re-oriented 4-point BCC noise" which constructs a
 *   congruent BCC lattice in a much different way than usual.
 * - 4D constructs the lattice as a union of five copies of its
 *   reciprocal. It successively finds the closest point on each.
 *
 * Multiple versions of each function are provided. See the
 * documentation above each, for more info.
 */
#[derive(Clone)]
pub struct OpenSimplex2F {
    perm: Vec<i16>,
    permGrad2: Vec<Grad2>,
    permGrad3: Vec<Grad3>,
    permGrad4: Vec<Grad4>,
}

impl OpenSimplex2F {
    /**
     * Construct a new instance with the given seed.
     */
    pub fn new(mut seed: i64) -> Self {
        let mut perm = vec![Default::default(); PSIZE as usize];
        let mut permGrad2 = vec![Default::default(); PSIZE as usize];
        let mut permGrad3 = vec![Default::default(); PSIZE as usize];
        let mut permGrad4 = vec![Default::default(); PSIZE as usize];
        let mut source: Vec<i16> = (0 .. PSIZE as i16).collect::<Vec<_>>();
        let staticData = get_static_data();
        
        for i in (0 .. PSIZE as i64).rev() {
            seed = seed.overflowing_mul(6364136223846793005).0.overflowing_add(1442695040888963407).0;
            let mut r: i32 = ((seed + 31i64) % (i + 1i64)) as i32;
            if r < 0 { r += i as i32 + 1; }
            perm[i as usize] = source[r as usize];
            permGrad2[i as usize] = staticData.gradients_2d[perm[i as usize] as usize];
            permGrad3[i as usize] = staticData.gradients_3d[perm[i as usize] as usize];
            permGrad4[i as usize] = staticData.gradients_4d[perm[i as usize] as usize];
            source[r as usize] = source[i as usize];
        }
        
        Self { perm, permGrad2, permGrad3, permGrad4 }
    }
    
    /*
     * Noise Evaluators
     */
    
    /**
     * 2D Simplex noise, standard lattice orientation.
     */
    pub fn noise2(&self, x: f64, y: f64) -> f64 {
        
        // Get points for A2* lattice
        let s: f64 = 0.366025403784439 * (x + y);
        let xs: f64 = x + s;
        let ys = y + s;
        
        self.noise2_Base(xs, ys)
    }
    
    /**
     * 2D Simplex noise, with Y pointing down the main diagonal.
     * Might be better for a 2D sandbox style game, where Y is vertical.
     * Probably slightly less optimal for heightmaps or continent maps.
     */
    pub fn noise2_XBeforeY(&self, x: f64, y: f64) -> f64 {
        
        // Skew transform and rotation baked into one.
        let xx = x * 0.7071067811865476;
        let yy = y * 1.224744871380249;
        
        self.noise2_Base(yy + xx, yy - xx)
    }
    
    /*
     * 2D Simplex noise base.
     * Lookup table implementation inspired by DigitalShadow.
     */
    fn noise2_Base(&self, xs: f64, ys: f64) -> f64 {
        let mut value = 0f64;
        
        // Get base points and offsets
        let xsb = f64::floor(xs) as i32;
        let ysb = f64::floor(ys) as i32;
        let xsi = xs - xsb as f64;
        let ysi = ys - ysb as f64;
        
        // Index to point list
        let index = ((ysi - xsi) / 2.0 + 1.0) as usize;
        
        let ssi = (xsi + ysi) * -0.211324865405187;
        let xi = xsi + ssi;
        let yi = ysi + ssi;
        let staticData = get_static_data();

        // Point contributions
        for i in 0 .. 3 {
            let c = staticData.lookup_2d[index + i];

            let dx = xi + c.dx;
            let dy = yi + c.dy;
            let attn = 0.5 - dx * dx - dy * dy;
            if attn <= 0.0 { continue; }

            let pxm = (xsb + c.xsv) & PMASK;
            let pym = (ysb + c.ysv) & PMASK;
            let grad = self.permGrad2[(self.perm[pxm as usize] as i32 ^ pym) as usize];
            let extrapolation = grad.dx * dx + grad.dy * dy;
            
            let attn = attn * attn;
            value += attn * attn * extrapolation;
        }
        
        value
    }
    
    /**
     * 3D Re-oriented 4-point BCC noise, classic orientation.
     * Proper substitute for 3D Simplex in light of Forbidden Formulae.
     * Use noise3_XYBeforeZ or noise3_XZBeforeY instead, wherever appropriate.
     */
    pub fn noise3_Classic(&self, x: f64, y: f64, z: f64) -> f64 {
        
        // Re-orient the cubic lattices via rotation, to produce the expected look on cardinal planar slices.
        // If texturing objects that don't tend to have cardinal plane faces, you could even remove this.
        // Orthonormal rotation. Not a skew transform.
        let r = (2.0 / 3.0) * (x + y + z);
        let xr = r - x;
        let yr = r - y;
        let zr = r - z;
        
        // Evaluate both lattices to form a BCC lattice.
        self.noise3_BCC(xr, yr, zr)
    }
    
    /**
     * 3D Re-oriented 4-point BCC noise, with better visual isotropy in (X, Y).
     * Recommended for 3D terrain and time-varied animations.
     * The Z coordinate should always be the "different" coordinate in your use case.
     * If Y is vertical in world coordinates, call noise3_XYBeforeZ(x, z, Y) or use noise3_XZBeforeY.
     * If Z is vertical in world coordinates, call noise3_XYBeforeZ(x, y, Z).
     * For a time varied animation, call noise3_XYBeforeZ(x, y, T).
     */
    pub fn noise3_XYBeforeZ(&self, x: f64, y: f64, z: f64) -> f64 {
        
        // Re-orient the cubic lattices without skewing, to make X and Y triangular like 2D.
        // Orthonormal rotation. Not a skew transform.
        let xy = x + y;
        let s2 = xy * -0.211324865405187;
        let zz = z * 0.577350269189626;
        let xr = x + s2 - zz;
        let yr = y + s2 - zz;
        let zr = xy * 0.577350269189626 + zz;
        
        // Evaluate both lattices to form a BCC lattice.
        self.noise3_BCC(xr, yr, zr)
    }
    
    /**
     * 3D Re-oriented 4-point BCC noise, with better visual isotropy in (X, Z).
     * Recommended for 3D terrain and time-varied animations.
     * The Y coordinate should always be the "different" coordinate in your use case.
     * If Y is vertical in world coordinates, call noise3_XZBeforeY(x, Y, z).
     * If Z is vertical in world coordinates, call noise3_XZBeforeY(x, Z, y) or use noise3_XYBeforeZ.
     * For a time varied animation, call noise3_XZBeforeY(x, T, y) or use noise3_XYBeforeZ.
     */
    pub fn noise3_XZBeforeY(&self, x: f64, y: f64, z: f64) -> f64 {
        
        // Re-orient the cubic lattices without skewing, to make X and Z triangular like 2D.
        // Orthonormal rotation. Not a skew transform.
        let xz = x + z;
        let s2 = xz * -0.211324865405187;
        let yy = y * 0.577350269189626;
        let xr = x + s2 - yy;
        let zr = z + s2 - yy;
        let yr = xz * 0.577350269189626 + yy;
        
        // Evaluate both lattices to form a BCC lattice.
        self.noise3_BCC(xr, yr, zr)
    }
    
    /*
     * Generate overlapping cubic lattices for 3D Re-oriented BCC noise.
     * Lookup table implementation inspired by DigitalShadow.
     * It was actually faster to narrow down the points in the loop itself,
     * than to build up the index with enough info to isolate 4 points.
     */
    fn noise3_BCC(&self, xr: f64, yr: f64, zr: f64) -> f64 {
        
        // Get base and offsets inside cube of first lattice.
        let xrb: i32 = f64::floor(xr) as i32;
        let yrb: i32 = f64::floor(yr) as i32;
        let zrb: i32 = f64::floor(zr) as i32;
        let xri: f64 = xr - xrb as f64;
        let yri: f64 = yr - yrb as f64;
        let zri: f64 = zr - zrb as f64;
        
        // Identify which octant of the cube we're in. This determines which cell
        // in the other cubic lattice we're in, and also narrows down one point on each.
        let xht: i32 = (xri + 0.5) as i32;
        let yht: i32 = (yri + 0.5) as i32;
        let zht: i32 = (zri + 0.5) as i32;
        let index = ((xht << 0) | (yht << 1) | (zht << 2)) as usize;
        let staticData = get_static_data();
        
        // Point contributions
        let mut value = 0.0;
        let mut curr: Option<LatticePoint3D> = Some(staticData.lookup_3d[index].clone());
        while curr.is_some() {
            let c = curr.unwrap();
            let dxr = xri + c.dxr;
            let dyr = yri + c.dyr;
            let dzr = zri + c.dzr;
            let attn = 0.5 - dxr * dxr - dyr * dyr - dzr * dzr;
            if attn < 0.0 {
                curr = c.nextOnFailure.map(|v| *v);
            } else {
                let pxm = ((xrb + c.xrv) & PMASK) as i16;
                let pym = ((yrb + c.yrv) & PMASK) as i16;
                let pzm = ((zrb + c.zrv) & PMASK) as i16;
                let grad = self.permGrad3[(self.perm[(self.perm[pxm as usize] ^ pym) as usize] ^ pzm) as usize];
                let extrapolation = grad.dx * dxr + grad.dy * dyr + grad.dz * dzr;
                
                let attn = attn * attn;
                value += attn * attn * extrapolation;
                curr = c.nextOnSuccess.map(|v| *v);
            }
        }
        
        value
    }
    
    /**
     * 4D OpenSimplex2F noise, classic lattice orientation.
     */
    pub fn noise4_Classic(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        
        // Get points for A4 lattice
        let s = -0.138196601125011 * (x + y + z + w);
        let xs = x + s;
        let ys = y + s;
        let zs = z + s;
        let ws = w + s;
        
        self.noise4_Base(xs, ys, zs, ws)
    }
    
    /**
     * 4D OpenSimplex2F noise, with XY and ZW forming orthogonal triangular-based planes.
     * Recommended for 3D terrain, where X and Y (or Z and W) are horizontal.
     * Recommended for noise(x, y, sin(time), cos(time)) trick.
     */
    pub fn noise4_XYBeforeZW(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        
        let s2 = (x + y) * -0.178275657951399372 + (z + w) * 0.215623393288842828;
        let t2 = (z + w) * -0.403949762580207112 + (x + y) * -0.375199083010075342;
        let xs = x + s2;
        let ys = y + s2;
        let zs = z + t2;
        let ws = w + t2;
        
        self.noise4_Base(xs, ys, zs, ws)
    }
    
    /**
     * 4D OpenSimplex2F noise, with XZ and YW forming orthogonal triangular-based planes.
     * Recommended for 3D terrain, where X and Z (or Y and W) are horizontal.
     */
    pub fn noise4_XZBeforeYW(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        
        let s2 = (x + z) * -0.178275657951399372 + (y + w) * 0.215623393288842828;
        let t2 = (y + w) * -0.403949762580207112 + (x + z) * -0.375199083010075342;
        let xs = x + s2;
        let ys = y + t2;
        let zs = z + s2;
        let ws = w + t2;
        
        self.noise4_Base(xs, ys, zs, ws)
    }
    
    /**
     * 4D OpenSimplex2F noise, with XYZ oriented like noise3_Classic,
     * and W for an extra degree of freedom. W repeats eventually.
     * Recommended for time-varied animations which texture a 3D object (W=time)
     */
    pub fn noise4_XYZBeforeW(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
        
        let xyz = x + y + z;
        let ww = w * 0.2236067977499788;
        let s2 = xyz * -0.16666666666666666 + ww;
        let xs = x + s2;
        let ys = y + s2;
        let zs = z + s2;
        let ws = -0.5 * xyz + ww;
        
        self.noise4_Base(xs, ys, zs, ws)
    }
    
    /*
     * 4D OpenSimplex2F noise base.
     * Current implementation not fully optimized by lookup tables.
     * But still comes out slightly ahead of Gustavson's Simplex in tests.
     */
    fn noise4_Base(&self, xs: f64, ys: f64, zs: f64, ws: f64) -> f64 {
        let mut value: f64 = 0.0;
        
        // Get base points and offsets
        let mut xsb = f64::floor(xs) as i32;
        let mut ysb = f64::floor(ys) as i32;
        let mut zsb = f64::floor(zs) as i32;
        let mut wsb = f64::floor(ws) as i32;
        let mut xsi = xs - xsb as f64;
        let mut ysi = ys - ysb as f64;
        let mut zsi = zs - zsb as f64;
        let mut wsi = ws - wsb as f64;
        
        // If we're in the lower half, flip so we can repeat the code for the upper half. We'll flip back later.
        let mut siSum = xsi + ysi + zsi + wsi;
        let mut ssi = siSum * 0.309016994374947; // Prep for vertex contributions.
        let inLowerHalf = siSum < 2.0;
        if inLowerHalf {
            xsi = 1.0 - xsi;
            ysi = 1.0 - ysi;
            zsi = 1.0 - zsi;
            wsi = 1.0 - wsi;
            siSum = 4.0 - siSum;
        }
        
        // Consider opposing vertex pairs of the octahedron formed by the central cross-section of the stretched tesseract
        let aabb = xsi + ysi - zsi - wsi;
        let abab = xsi - ysi + zsi - wsi;
        let abba = xsi - ysi - zsi + wsi;
        let aabbScore = f64::abs(aabb);
        let ababScore = f64::abs(abab);
        let abbaScore = f64::abs(abba);
        
        // Find the closest point on the stretched tesseract as if it were the upper half
        let mut vertexIndex: i32;
        let mut via: i32;
        let vib: i32;
        let mut asi: f64;
        let mut bsi: f64;
        if aabbScore > ababScore && aabbScore > abbaScore {
            if aabb > 0.0 {
                asi = zsi;
                bsi = wsi;
                vertexIndex = 0b0011;
                via = 0b0111;
                vib = 0b1011;
            } else {
                asi = xsi;
                bsi = ysi;
                vertexIndex = 0b1100;
                via = 0b1101;
                vib = 0b1110;
            }
        } else if ababScore > abbaScore {
            if abab > 0.0 {
                asi = ysi;
                bsi = wsi;
                vertexIndex = 0b0101;
                via = 0b0111;
                vib = 0b1101;
            } else {
                asi = xsi;
                bsi = zsi;
                vertexIndex = 0b1010;
                via = 0b1011;
                vib = 0b1110;
            }
        } else {
            if abba > 0.0 {
                asi = ysi;
                bsi = zsi;
                vertexIndex = 0b1001;
                via = 0b1011;
                vib = 0b1101;
            } else {
                asi = xsi;
                bsi = wsi;
                vertexIndex = 0b0110;
                via = 0b0111;
                vib = 0b1110;
            }
        }
        if bsi > asi {
            via = vib;
            std::mem::swap(&mut asi, &mut bsi);
        }
        if siSum + asi > 3.0 {
            vertexIndex = via;
            if siSum + bsi > 4.0 {
                vertexIndex = 0b1111;
            }
        }
        
        // Now flip back if we're actually in the lower half.
        if inLowerHalf {
            xsi = 1.0 - xsi;
            ysi = 1.0 - ysi;
            zsi = 1.0 - zsi;
            wsi = 1.0 - wsi;
            vertexIndex ^= 0b1111;
        }
        
        let staticData = get_static_data();
        
        // Five points to add, total, from five copies of the A4 lattice.
        for i in 0 .. 5 {
        
            // Update xsb/etc. and add the lattice point's contribution.
            let c = staticData.vertices_4d[vertexIndex as usize];
            xsb += c.xsv;
            ysb += c.ysv;
            zsb += c.zsv;
            wsb += c.wsv;
            let xi = xsi + ssi;
            let yi = ysi + ssi;
            let zi = zsi + ssi;
            let wi = wsi + ssi;
            let dx = xi + c.dx;
            let dy = yi + c.dy;
            let dz = zi + c.dz;
            let dw = wi + c.dw;
            let attn = 0.5 - dx * dx - dy * dy - dz * dz - dw * dw;
            if attn > 0.0 {
                let pxm = xsb & PMASK;
                let pym = ysb & PMASK;
                let pzm = zsb & PMASK;
                let pwm = wsb & PMASK;
                let grad = self.permGrad4[(self.perm[(self.perm[(self.perm[pxm as usize] ^ pym as i16) as usize] ^ pzm as i16) as usize] ^ pwm as i16) as usize];
                let ramped = grad.dx * dx + grad.dy * dy + grad.dz * dz + grad.dw * dw;
                
                let attn = attn * attn;
                value += attn * attn * ramped;
            }
            
            // Maybe this helps the compiler/JVM/LLVM/etc. know we can end the loop here. Maybe not.
            if i == 4 { break; }
            
            // Update the relative skewed coordinates to reference the vertex we just added.
            // Rather, reference its counterpart on the lattice copy that is shifted down by
            // the vector <-0.2, -0.2, -0.2, -0.2>
            xsi += c.xsi;
            ysi += c.ysi;
            zsi += c.zsi;
            wsi += c.wsi;
            ssi += c.ssiDelta;
            
            // Next point is the closest vertex on the 4-simplex whose base vertex is the aforementioned vertex.
            let score0 = 1.0 + ssi * (-1.0 / 0.309016994374947); // Seems slightly faster than 1.0-xsi-ysi-zsi-wsi
            vertexIndex = 0b0000;
            if xsi >= ysi && xsi >= zsi && xsi >= wsi && xsi >= score0 {
                vertexIndex = 0b0001;
            }
            else if ysi > xsi && ysi >= zsi && ysi >= wsi && ysi >= score0 {
                vertexIndex = 0b0010;
            }
            else if zsi > xsi && zsi > ysi && zsi >= wsi && zsi >= score0 {
                vertexIndex = 0b0100;
            }
            else if wsi > xsi && wsi > ysi && wsi > zsi && wsi >= score0 {
                vertexIndex = 0b1000;
            }
        }
        
        value
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Grad2 {
    dx: f64,
    dy: f64,
}

impl Grad2 {
    fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Grad3 {
    dx: f64,
    dy: f64,
    dz: f64,
}

impl Grad3 {
    fn new(dx: f64, dy: f64, dz: f64) -> Self {
        Self { dx, dy, dz }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Grad4 {
    dx: f64,
    dy: f64,
    dz: f64,
    dw: f64,
}

impl Grad4 {
    fn new(dx: f64, dy: f64, dz: f64, dw: f64) -> Self {
        Self { dx, dy, dz, dw }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct LatticePoint2D {
    xsv: i32,
    ysv: i32,
    dx: f64,
    dy: f64,
}

impl LatticePoint2D {
    fn new(xsv: i32, ysv: i32) -> Self {
        let ssv: f64 = (xsv + ysv) as f64 * -0.211324865405187;
        Self {
            xsv, ysv,
            dx: -(xsv as f64) - ssv,
            dy: -(ysv as f64) - ssv,
        }
    }
}

#[derive(Clone, Default, Debug)]
struct LatticePoint3D {
    dxr: f64,
    dyr: f64,
    dzr: f64,
    xrv: i32,
    yrv: i32,
    zrv: i32,
    nextOnFailure: Option<Box<LatticePoint3D>>, // *facepalm*
    nextOnSuccess: Option<Box<LatticePoint3D>>,
}

impl LatticePoint3D {
    fn new(xrv: i32, yrv: i32, zrv: i32, lattice: i32) -> Self {
        Self {
            dxr: (-xrv as f64) + lattice as f64 * 0.5,
            dyr: (-yrv as f64) + lattice as f64 * 0.5,
            dzr: (-zrv as f64) + lattice as f64 * 0.5,
            xrv: xrv + lattice * 1024,
            yrv: yrv + lattice * 1024,
            zrv: zrv + lattice * 1024,
            nextOnFailure: None,
            nextOnSuccess: None,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct LatticePoint4D {
    xsv: i32,
    ysv: i32,
    zsv: i32,
    wsv: i32,
    dx: f64,
    dy: f64,
    dz: f64,
    dw: f64,
    xsi: f64,
    ysi: f64,
    zsi: f64,
    wsi: f64,
    ssiDelta: f64,
}

impl LatticePoint4D {
    fn new(xsv: i32, ysv: i32, zsv: i32, wsv: i32) -> Self {
        let ssv: f64 = (xsv + ysv + zsv + wsv) as f64 * 0.309016994374947;
        Self {
            xsv: xsv + 409,
            ysv: ysv + 409,
            zsv: zsv + 409,
            wsv: wsv + 409,
            dx: -(xsv as f64) - ssv,
            dy: -(ysv as f64) - ssv,
            dz: -(zsv as f64) - ssv,
            dw: -(wsv as f64) - ssv,
            xsi: 0.2 - (xsv as f64),
            ysi: 0.2 - (ysv as f64),
            zsi: 0.2 - (zsv as f64),
            wsi: 0.2 - (wsv as f64),
            ssiDelta: (0.8 - (xsv as f64) - (ysv as f64) - (zsv as f64) - (wsv as f64)) * 0.309016994374947,
        }
    }
}

const PSIZE: i32 = 2048;
const PMASK: i32 = 2047;
static mut LOOKUP_2D: Vec<LatticePoint2D> = vec![];
static mut LOOKUP_3D: Vec<LatticePoint3D> = vec![];
static mut VERTICES_4D: Vec<LatticePoint4D> = vec![];

unsafe fn init_lattice_points() {
    LOOKUP_2D = vec![
        LatticePoint2D::new(1, 0),
        LatticePoint2D::new(0, 0),
        LatticePoint2D::new(1, 1),
        LatticePoint2D::new(0, 1),
    ];
    
    LOOKUP_3D = vec![Default::default(); 8];
    
    for i in 0 .. LOOKUP_3D.len() as i32 {
        let i1: i32 = (i >> 0) & 1;
        let j1: i32 = (i >> 1) & 1;
        let k1: i32 = (i >> 2) & 1;
        let i2: i32 = i1 ^ 1;
        let j2: i32 = j1 ^ 1;
        let k2: i32 = k1 ^ 1;
        
        // The two points within this octant, one from each of the two cubic half-lattices.
        let mut c0: LatticePoint3D = LatticePoint3D::new(i1, j1, k1, 0);
        let mut c1: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1 + i2, j1 + j2, k1 + k2, 1));
        
        // Each single step away on the first half-lattice.
        let mut c2: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1 ^ 1, j1, k1, 0));
        let mut c3: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1, j1 ^ 1, k1, 0));
        let mut c4: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1, j1, k1 ^ 1, 0));
        
        // Each single step away on the second half-lattice.
        let mut c5: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1 + (i2 ^ 1), j1 + j2, k1 + k2, 1));
        let mut c6: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1 + i2, j1 + (j2 ^ 1), k1 + k2, 1));
        let c7: Box<LatticePoint3D> = Box::new(LatticePoint3D::new(i1 + i2, j1 + j2, k1 + (k2 ^ 1), 1));
        
        // porting note: reversed to work with cloning
        c6.nextOnFailure = Some(c7.clone());
        
        c5.nextOnFailure = Some(c6.clone());
        
        c4.nextOnFailure = Some(c5.clone());
        c4.nextOnSuccess = Some(c5.clone());
        
        c3.nextOnFailure = Some(c4.clone());
        c3.nextOnSuccess = Some(c5.clone());
        
        c2.nextOnFailure = Some(c3.clone());
        c2.nextOnSuccess = Some(c6.clone());
        
        c1.nextOnFailure = Some(c2.clone());
        c1.nextOnSuccess = Some(c2.clone());
        
        c0.nextOnFailure = Some(c1.clone());
        c0.nextOnSuccess = Some(c1.clone());
        
        LOOKUP_3D[i as usize] = c0;
    }
    
    VERTICES_4D = vec![Default::default(); 16];
    
    for i in 0 .. VERTICES_4D.len() as i32 {
        VERTICES_4D[i as usize] = LatticePoint4D::new((i >> 0) & 1, (i >> 1) & 1, (i >> 2) & 1, (i >> 3) & 1);
    }
}

const N2: f64 = 0.01001634121365712;
const N3: f64 = 0.030485933181293584;
const N4: f64 = 0.009202377986303158;
static mut GRADIENTS_2D: Vec<Grad2> = vec![];
static mut GRADIENTS_3D: Vec<Grad3> = vec![];
static mut GRADIENTS_4D: Vec<Grad4> = vec![];

unsafe fn init_gradients() {
    GRADIENTS_2D = vec![Default::default(); PSIZE as usize];
    let mut grad2 = vec![
        Grad2::new( 0.130526192220052,  0.99144486137381),
        Grad2::new( 0.38268343236509,   0.923879532511287),
        Grad2::new( 0.608761429008721,  0.793353340291235),
        Grad2::new( 0.793353340291235,  0.608761429008721),
        Grad2::new( 0.923879532511287,  0.38268343236509),
        Grad2::new( 0.99144486137381,   0.130526192220051),
        Grad2::new( 0.99144486137381,  -0.130526192220051),
        Grad2::new( 0.923879532511287, -0.38268343236509),
        Grad2::new( 0.793353340291235, -0.60876142900872),
        Grad2::new( 0.608761429008721, -0.793353340291235),
        Grad2::new( 0.38268343236509,  -0.923879532511287),
        Grad2::new( 0.130526192220052, -0.99144486137381),
        Grad2::new(-0.130526192220052, -0.99144486137381),
        Grad2::new(-0.38268343236509,  -0.923879532511287),
        Grad2::new(-0.608761429008721, -0.793353340291235),
        Grad2::new(-0.793353340291235, -0.608761429008721),
        Grad2::new(-0.923879532511287, -0.38268343236509),
        Grad2::new(-0.99144486137381,  -0.130526192220052),
        Grad2::new(-0.99144486137381,   0.130526192220051),
        Grad2::new(-0.923879532511287,  0.38268343236509),
        Grad2::new(-0.793353340291235,  0.608761429008721),
        Grad2::new(-0.608761429008721,  0.793353340291235),
        Grad2::new(-0.38268343236509,   0.923879532511287),
        Grad2::new(-0.130526192220052,  0.99144486137381)
    ];
    for i in 0 .. grad2.len() {
        grad2[i as usize].dx /= N2;
        grad2[i as usize].dy /= N2;
    }
    for i in 0 .. GRADIENTS_2D.len() {
        GRADIENTS_2D[i as usize] = grad2[i as usize % grad2.len()];
    }
    
    GRADIENTS_3D = vec![Default::default(); PSIZE as usize];
    let mut grad3 = vec![
        Grad3::new(-2.22474487139,      -2.22474487139,      -1.0),
        Grad3::new(-2.22474487139,      -2.22474487139,       1.0),
        Grad3::new(-3.0862664687972017, -1.1721513422464978,  0.0),
        Grad3::new(-1.1721513422464978, -3.0862664687972017,  0.0),
        Grad3::new(-2.22474487139,      -1.0,                -2.22474487139),
        Grad3::new(-2.22474487139,       1.0,                -2.22474487139),
        Grad3::new(-1.1721513422464978,  0.0,                -3.0862664687972017),
        Grad3::new(-3.0862664687972017,  0.0,                -1.1721513422464978),
        Grad3::new(-2.22474487139,      -1.0,                 2.22474487139),
        Grad3::new(-2.22474487139,       1.0,                 2.22474487139),
        Grad3::new(-3.0862664687972017,  0.0,                 1.1721513422464978),
        Grad3::new(-1.1721513422464978,  0.0,                 3.0862664687972017),
        Grad3::new(-2.22474487139,       2.22474487139,      -1.0),
        Grad3::new(-2.22474487139,       2.22474487139,       1.0),
        Grad3::new(-1.1721513422464978,  3.0862664687972017,  0.0),
        Grad3::new(-3.0862664687972017,  1.1721513422464978,  0.0),
        Grad3::new(-1.0,                -2.22474487139,      -2.22474487139),
        Grad3::new( 1.0,                -2.22474487139,      -2.22474487139),
        Grad3::new( 0.0,                -3.0862664687972017, -1.1721513422464978),
        Grad3::new( 0.0,                -1.1721513422464978, -3.0862664687972017),
        Grad3::new(-1.0,                -2.22474487139,       2.22474487139),
        Grad3::new( 1.0,                -2.22474487139,       2.22474487139),
        Grad3::new( 0.0,                -1.1721513422464978,  3.0862664687972017),
        Grad3::new( 0.0,                -3.0862664687972017,  1.1721513422464978),
        Grad3::new(-1.0,                 2.22474487139,      -2.22474487139),
        Grad3::new( 1.0,                 2.22474487139,      -2.22474487139),
        Grad3::new( 0.0,                 1.1721513422464978, -3.0862664687972017),
        Grad3::new( 0.0,                 3.0862664687972017, -1.1721513422464978),
        Grad3::new(-1.0,                 2.22474487139,       2.22474487139),
        Grad3::new( 1.0,                 2.22474487139,       2.22474487139),
        Grad3::new( 0.0,                 3.0862664687972017,  1.1721513422464978),
        Grad3::new( 0.0,                 1.1721513422464978,  3.0862664687972017),
        Grad3::new( 2.22474487139,      -2.22474487139,      -1.0),
        Grad3::new( 2.22474487139,      -2.22474487139,       1.0),
        Grad3::new( 1.1721513422464978, -3.0862664687972017,  0.0),
        Grad3::new( 3.0862664687972017, -1.1721513422464978,  0.0),
        Grad3::new( 2.22474487139,      -1.0,                -2.22474487139),
        Grad3::new( 2.22474487139,       1.0,                -2.22474487139),
        Grad3::new( 3.0862664687972017,  0.0,                -1.1721513422464978),
        Grad3::new( 1.1721513422464978,  0.0,                -3.0862664687972017),
        Grad3::new( 2.22474487139,      -1.0,                 2.22474487139),
        Grad3::new( 2.22474487139,       1.0,                 2.22474487139),
        Grad3::new( 1.1721513422464978,  0.0,                 3.0862664687972017),
        Grad3::new( 3.0862664687972017,  0.0,                 1.1721513422464978),
        Grad3::new( 2.22474487139,       2.22474487139,      -1.0),
        Grad3::new( 2.22474487139,       2.22474487139,       1.0),
        Grad3::new( 3.0862664687972017,  1.1721513422464978,  0.0),
        Grad3::new( 1.1721513422464978,  3.0862664687972017,  0.0)
    ];
    for i in 0.. grad3.len() {
        grad3[i as usize].dx /= N3;
        grad3[i as usize].dy /= N3;
        grad3[i as usize].dz /= N3;
    }
    for i in 0 .. GRADIENTS_3D.len() {
        GRADIENTS_3D[i as usize] = grad3[i as usize % grad3.len()];
    }
    
    GRADIENTS_4D = vec![Default::default(); PSIZE as usize];
    let mut grad4 = vec![
        Grad4::new(-0.753341017856078,    -0.37968289875261624,  -0.37968289875261624,  -0.37968289875261624),
        Grad4::new(-0.7821684431180708,   -0.4321472685365301,   -0.4321472685365301,    0.12128480194602098),
        Grad4::new(-0.7821684431180708,   -0.4321472685365301,    0.12128480194602098,  -0.4321472685365301),
        Grad4::new(-0.7821684431180708,    0.12128480194602098,  -0.4321472685365301,   -0.4321472685365301),
        Grad4::new(-0.8586508742123365,   -0.508629699630796,     0.044802370851755174,  0.044802370851755174),
        Grad4::new(-0.8586508742123365,    0.044802370851755174, -0.508629699630796,     0.044802370851755174),
        Grad4::new(-0.8586508742123365,    0.044802370851755174,  0.044802370851755174, -0.508629699630796),
        Grad4::new(-0.9982828964265062,   -0.03381941603233842,  -0.03381941603233842,  -0.03381941603233842),
        Grad4::new(-0.37968289875261624,  -0.753341017856078,    -0.37968289875261624,  -0.37968289875261624),
        Grad4::new(-0.4321472685365301,   -0.7821684431180708,   -0.4321472685365301,    0.12128480194602098),
        Grad4::new(-0.4321472685365301,   -0.7821684431180708,    0.12128480194602098,  -0.4321472685365301),
        Grad4::new( 0.12128480194602098,  -0.7821684431180708,   -0.4321472685365301,   -0.4321472685365301),
        Grad4::new(-0.508629699630796,    -0.8586508742123365,    0.044802370851755174,  0.044802370851755174),
        Grad4::new( 0.044802370851755174, -0.8586508742123365,   -0.508629699630796,     0.044802370851755174),
        Grad4::new( 0.044802370851755174, -0.8586508742123365,    0.044802370851755174, -0.508629699630796),
        Grad4::new(-0.03381941603233842,  -0.9982828964265062,   -0.03381941603233842,  -0.03381941603233842),
        Grad4::new(-0.37968289875261624,  -0.37968289875261624,  -0.753341017856078,    -0.37968289875261624),
        Grad4::new(-0.4321472685365301,   -0.4321472685365301,   -0.7821684431180708,    0.12128480194602098),
        Grad4::new(-0.4321472685365301,    0.12128480194602098,  -0.7821684431180708,   -0.4321472685365301),
        Grad4::new( 0.12128480194602098,  -0.4321472685365301,   -0.7821684431180708,   -0.4321472685365301),
        Grad4::new(-0.508629699630796,     0.044802370851755174, -0.8586508742123365,    0.044802370851755174),
        Grad4::new( 0.044802370851755174, -0.508629699630796,    -0.8586508742123365,    0.044802370851755174),
        Grad4::new( 0.044802370851755174,  0.044802370851755174, -0.8586508742123365,   -0.508629699630796),
        Grad4::new(-0.03381941603233842,  -0.03381941603233842,  -0.9982828964265062,   -0.03381941603233842),
        Grad4::new(-0.37968289875261624,  -0.37968289875261624,  -0.37968289875261624,  -0.753341017856078),
        Grad4::new(-0.4321472685365301,   -0.4321472685365301,    0.12128480194602098,  -0.7821684431180708),
        Grad4::new(-0.4321472685365301,    0.12128480194602098,  -0.4321472685365301,   -0.7821684431180708),
        Grad4::new( 0.12128480194602098,  -0.4321472685365301,   -0.4321472685365301,   -0.7821684431180708),
        Grad4::new(-0.508629699630796,     0.044802370851755174,  0.044802370851755174, -0.8586508742123365),
        Grad4::new( 0.044802370851755174, -0.508629699630796,     0.044802370851755174, -0.8586508742123365),
        Grad4::new( 0.044802370851755174,  0.044802370851755174, -0.508629699630796,    -0.8586508742123365),
        Grad4::new(-0.03381941603233842,  -0.03381941603233842,  -0.03381941603233842,  -0.9982828964265062),
        Grad4::new(-0.6740059517812944,   -0.3239847771997537,   -0.3239847771997537,    0.5794684678643381),
        Grad4::new(-0.7504883828755602,   -0.4004672082940195,    0.15296486218853164,   0.5029860367700724),
        Grad4::new(-0.7504883828755602,    0.15296486218853164,  -0.4004672082940195,    0.5029860367700724),
        Grad4::new(-0.8828161875373585,    0.08164729285680945,   0.08164729285680945,   0.4553054119602712),
        Grad4::new(-0.4553054119602712,   -0.08164729285680945,  -0.08164729285680945,   0.8828161875373585),
        Grad4::new(-0.5029860367700724,   -0.15296486218853164,   0.4004672082940195,    0.7504883828755602),
        Grad4::new(-0.5029860367700724,    0.4004672082940195,   -0.15296486218853164,   0.7504883828755602),
        Grad4::new(-0.5794684678643381,    0.3239847771997537,    0.3239847771997537,    0.6740059517812944),
        Grad4::new(-0.3239847771997537,   -0.6740059517812944,   -0.3239847771997537,    0.5794684678643381),
        Grad4::new(-0.4004672082940195,   -0.7504883828755602,    0.15296486218853164,   0.5029860367700724),
        Grad4::new( 0.15296486218853164,  -0.7504883828755602,   -0.4004672082940195,    0.5029860367700724),
        Grad4::new( 0.08164729285680945,  -0.8828161875373585,    0.08164729285680945,   0.4553054119602712),
        Grad4::new(-0.08164729285680945,  -0.4553054119602712,   -0.08164729285680945,   0.8828161875373585),
        Grad4::new(-0.15296486218853164,  -0.5029860367700724,    0.4004672082940195,    0.7504883828755602),
        Grad4::new( 0.4004672082940195,   -0.5029860367700724,   -0.15296486218853164,   0.7504883828755602),
        Grad4::new( 0.3239847771997537,   -0.5794684678643381,    0.3239847771997537,    0.6740059517812944),
        Grad4::new(-0.3239847771997537,   -0.3239847771997537,   -0.6740059517812944,    0.5794684678643381),
        Grad4::new(-0.4004672082940195,    0.15296486218853164,  -0.7504883828755602,    0.5029860367700724),
        Grad4::new( 0.15296486218853164,  -0.4004672082940195,   -0.7504883828755602,    0.5029860367700724),
        Grad4::new( 0.08164729285680945,   0.08164729285680945,  -0.8828161875373585,    0.4553054119602712),
        Grad4::new(-0.08164729285680945,  -0.08164729285680945,  -0.4553054119602712,    0.8828161875373585),
        Grad4::new(-0.15296486218853164,   0.4004672082940195,   -0.5029860367700724,    0.7504883828755602),
        Grad4::new( 0.4004672082940195,   -0.15296486218853164,  -0.5029860367700724,    0.7504883828755602),
        Grad4::new( 0.3239847771997537,    0.3239847771997537,   -0.5794684678643381,    0.6740059517812944),
        Grad4::new(-0.6740059517812944,   -0.3239847771997537,    0.5794684678643381,   -0.3239847771997537),
        Grad4::new(-0.7504883828755602,   -0.4004672082940195,    0.5029860367700724,    0.15296486218853164),
        Grad4::new(-0.7504883828755602,    0.15296486218853164,   0.5029860367700724,   -0.4004672082940195),
        Grad4::new(-0.8828161875373585,    0.08164729285680945,   0.4553054119602712,    0.08164729285680945),
        Grad4::new(-0.4553054119602712,   -0.08164729285680945,   0.8828161875373585,   -0.08164729285680945),
        Grad4::new(-0.5029860367700724,   -0.15296486218853164,   0.7504883828755602,    0.4004672082940195),
        Grad4::new(-0.5029860367700724,    0.4004672082940195,    0.7504883828755602,   -0.15296486218853164),
        Grad4::new(-0.5794684678643381,    0.3239847771997537,    0.6740059517812944,    0.3239847771997537),
        Grad4::new(-0.3239847771997537,   -0.6740059517812944,    0.5794684678643381,   -0.3239847771997537),
        Grad4::new(-0.4004672082940195,   -0.7504883828755602,    0.5029860367700724,    0.15296486218853164),
        Grad4::new( 0.15296486218853164,  -0.7504883828755602,    0.5029860367700724,   -0.4004672082940195),
        Grad4::new( 0.08164729285680945,  -0.8828161875373585,    0.4553054119602712,    0.08164729285680945),
        Grad4::new(-0.08164729285680945,  -0.4553054119602712,    0.8828161875373585,   -0.08164729285680945),
        Grad4::new(-0.15296486218853164,  -0.5029860367700724,    0.7504883828755602,    0.4004672082940195),
        Grad4::new( 0.4004672082940195,   -0.5029860367700724,    0.7504883828755602,   -0.15296486218853164),
        Grad4::new( 0.3239847771997537,   -0.5794684678643381,    0.6740059517812944,    0.3239847771997537),
        Grad4::new(-0.3239847771997537,   -0.3239847771997537,    0.5794684678643381,   -0.6740059517812944),
        Grad4::new(-0.4004672082940195,    0.15296486218853164,   0.5029860367700724,   -0.7504883828755602),
        Grad4::new( 0.15296486218853164,  -0.4004672082940195,    0.5029860367700724,   -0.7504883828755602),
        Grad4::new( 0.08164729285680945,   0.08164729285680945,   0.4553054119602712,   -0.8828161875373585),
        Grad4::new(-0.08164729285680945,  -0.08164729285680945,   0.8828161875373585,   -0.4553054119602712),
        Grad4::new(-0.15296486218853164,   0.4004672082940195,    0.7504883828755602,   -0.5029860367700724),
        Grad4::new( 0.4004672082940195,   -0.15296486218853164,   0.7504883828755602,   -0.5029860367700724),
        Grad4::new( 0.3239847771997537,    0.3239847771997537,    0.6740059517812944,   -0.5794684678643381),
        Grad4::new(-0.6740059517812944,    0.5794684678643381,   -0.3239847771997537,   -0.3239847771997537),
        Grad4::new(-0.7504883828755602,    0.5029860367700724,   -0.4004672082940195,    0.15296486218853164),
        Grad4::new(-0.7504883828755602,    0.5029860367700724,    0.15296486218853164,  -0.4004672082940195),
        Grad4::new(-0.8828161875373585,    0.4553054119602712,    0.08164729285680945,   0.08164729285680945),
        Grad4::new(-0.4553054119602712,    0.8828161875373585,   -0.08164729285680945,  -0.08164729285680945),
        Grad4::new(-0.5029860367700724,    0.7504883828755602,   -0.15296486218853164,   0.4004672082940195),
        Grad4::new(-0.5029860367700724,    0.7504883828755602,    0.4004672082940195,   -0.15296486218853164),
        Grad4::new(-0.5794684678643381,    0.6740059517812944,    0.3239847771997537,    0.3239847771997537),
        Grad4::new(-0.3239847771997537,    0.5794684678643381,   -0.6740059517812944,   -0.3239847771997537),
        Grad4::new(-0.4004672082940195,    0.5029860367700724,   -0.7504883828755602,    0.15296486218853164),
        Grad4::new( 0.15296486218853164,   0.5029860367700724,   -0.7504883828755602,   -0.4004672082940195),
        Grad4::new( 0.08164729285680945,   0.4553054119602712,   -0.8828161875373585,    0.08164729285680945),
        Grad4::new(-0.08164729285680945,   0.8828161875373585,   -0.4553054119602712,   -0.08164729285680945),
        Grad4::new(-0.15296486218853164,   0.7504883828755602,   -0.5029860367700724,    0.4004672082940195),
        Grad4::new( 0.4004672082940195,    0.7504883828755602,   -0.5029860367700724,   -0.15296486218853164),
        Grad4::new( 0.3239847771997537,    0.6740059517812944,   -0.5794684678643381,    0.3239847771997537),
        Grad4::new(-0.3239847771997537,    0.5794684678643381,   -0.3239847771997537,   -0.6740059517812944),
        Grad4::new(-0.4004672082940195,    0.5029860367700724,    0.15296486218853164,  -0.7504883828755602),
        Grad4::new( 0.15296486218853164,   0.5029860367700724,   -0.4004672082940195,   -0.7504883828755602),
        Grad4::new( 0.08164729285680945,   0.4553054119602712,    0.08164729285680945,  -0.8828161875373585),
        Grad4::new(-0.08164729285680945,   0.8828161875373585,   -0.08164729285680945,  -0.4553054119602712),
        Grad4::new(-0.15296486218853164,   0.7504883828755602,    0.4004672082940195,   -0.5029860367700724),
        Grad4::new( 0.4004672082940195,    0.7504883828755602,   -0.15296486218853164,  -0.5029860367700724),
        Grad4::new( 0.3239847771997537,    0.6740059517812944,    0.3239847771997537,   -0.5794684678643381),
        Grad4::new( 0.5794684678643381,   -0.6740059517812944,   -0.3239847771997537,   -0.3239847771997537),
        Grad4::new( 0.5029860367700724,   -0.7504883828755602,   -0.4004672082940195,    0.15296486218853164),
        Grad4::new( 0.5029860367700724,   -0.7504883828755602,    0.15296486218853164,  -0.4004672082940195),
        Grad4::new( 0.4553054119602712,   -0.8828161875373585,    0.08164729285680945,   0.08164729285680945),
        Grad4::new( 0.8828161875373585,   -0.4553054119602712,   -0.08164729285680945,  -0.08164729285680945),
        Grad4::new( 0.7504883828755602,   -0.5029860367700724,   -0.15296486218853164,   0.4004672082940195),
        Grad4::new( 0.7504883828755602,   -0.5029860367700724,    0.4004672082940195,   -0.15296486218853164),
        Grad4::new( 0.6740059517812944,   -0.5794684678643381,    0.3239847771997537,    0.3239847771997537),
        Grad4::new( 0.5794684678643381,   -0.3239847771997537,   -0.6740059517812944,   -0.3239847771997537),
        Grad4::new( 0.5029860367700724,   -0.4004672082940195,   -0.7504883828755602,    0.15296486218853164),
        Grad4::new( 0.5029860367700724,    0.15296486218853164,  -0.7504883828755602,   -0.4004672082940195),
        Grad4::new( 0.4553054119602712,    0.08164729285680945,  -0.8828161875373585,    0.08164729285680945),
        Grad4::new( 0.8828161875373585,   -0.08164729285680945,  -0.4553054119602712,   -0.08164729285680945),
        Grad4::new( 0.7504883828755602,   -0.15296486218853164,  -0.5029860367700724,    0.4004672082940195),
        Grad4::new( 0.7504883828755602,    0.4004672082940195,   -0.5029860367700724,   -0.15296486218853164),
        Grad4::new( 0.6740059517812944,    0.3239847771997537,   -0.5794684678643381,    0.3239847771997537),
        Grad4::new( 0.5794684678643381,   -0.3239847771997537,   -0.3239847771997537,   -0.6740059517812944),
        Grad4::new( 0.5029860367700724,   -0.4004672082940195,    0.15296486218853164,  -0.7504883828755602),
        Grad4::new( 0.5029860367700724,    0.15296486218853164,  -0.4004672082940195,   -0.7504883828755602),
        Grad4::new( 0.4553054119602712,    0.08164729285680945,   0.08164729285680945,  -0.8828161875373585),
        Grad4::new( 0.8828161875373585,   -0.08164729285680945,  -0.08164729285680945,  -0.4553054119602712),
        Grad4::new( 0.7504883828755602,   -0.15296486218853164,   0.4004672082940195,   -0.5029860367700724),
        Grad4::new( 0.7504883828755602,    0.4004672082940195,   -0.15296486218853164,  -0.5029860367700724),
        Grad4::new( 0.6740059517812944,    0.3239847771997537,    0.3239847771997537,   -0.5794684678643381),
        Grad4::new( 0.03381941603233842,   0.03381941603233842,   0.03381941603233842,   0.9982828964265062),
        Grad4::new(-0.044802370851755174, -0.044802370851755174,  0.508629699630796,     0.8586508742123365),
        Grad4::new(-0.044802370851755174,  0.508629699630796,    -0.044802370851755174,  0.8586508742123365),
        Grad4::new(-0.12128480194602098,   0.4321472685365301,    0.4321472685365301,    0.7821684431180708),
        Grad4::new( 0.508629699630796,    -0.044802370851755174, -0.044802370851755174,  0.8586508742123365),
        Grad4::new( 0.4321472685365301,   -0.12128480194602098,   0.4321472685365301,    0.7821684431180708),
        Grad4::new( 0.4321472685365301,    0.4321472685365301,   -0.12128480194602098,   0.7821684431180708),
        Grad4::new( 0.37968289875261624,   0.37968289875261624,   0.37968289875261624,   0.753341017856078),
        Grad4::new( 0.03381941603233842,   0.03381941603233842,   0.9982828964265062,    0.03381941603233842),
        Grad4::new(-0.044802370851755174,  0.044802370851755174,  0.8586508742123365,    0.508629699630796),
        Grad4::new(-0.044802370851755174,  0.508629699630796,     0.8586508742123365,   -0.044802370851755174),
        Grad4::new(-0.12128480194602098,   0.4321472685365301,    0.7821684431180708,    0.4321472685365301),
        Grad4::new( 0.508629699630796,    -0.044802370851755174,  0.8586508742123365,   -0.044802370851755174),
        Grad4::new( 0.4321472685365301,   -0.12128480194602098,   0.7821684431180708,    0.4321472685365301),
        Grad4::new( 0.4321472685365301,    0.4321472685365301,    0.7821684431180708,   -0.12128480194602098),
        Grad4::new( 0.37968289875261624,   0.37968289875261624,   0.753341017856078,     0.37968289875261624),
        Grad4::new( 0.03381941603233842,   0.9982828964265062,    0.03381941603233842,   0.03381941603233842),
        Grad4::new(-0.044802370851755174,  0.8586508742123365,   -0.044802370851755174,  0.508629699630796),
        Grad4::new(-0.044802370851755174,  0.8586508742123365,    0.508629699630796,    -0.044802370851755174),
        Grad4::new(-0.12128480194602098,   0.7821684431180708,    0.4321472685365301,    0.4321472685365301),
        Grad4::new( 0.508629699630796,     0.8586508742123365,   -0.044802370851755174, -0.044802370851755174),
        Grad4::new( 0.4321472685365301,    0.7821684431180708,   -0.12128480194602098,   0.4321472685365301),
        Grad4::new( 0.4321472685365301,    0.7821684431180708,    0.4321472685365301,   -0.12128480194602098),
        Grad4::new( 0.37968289875261624,   0.753341017856078,     0.37968289875261624,   0.37968289875261624),
        Grad4::new( 0.9982828964265062,    0.03381941603233842,   0.03381941603233842,   0.03381941603233842),
        Grad4::new( 0.8586508742123365,   -0.044802370851755174, -0.044802370851755174,  0.508629699630796),
        Grad4::new( 0.8586508742123365,   -0.044802370851755174,  0.508629699630796,    -0.044802370851755174),
        Grad4::new( 0.7821684431180708,   -0.12128480194602098,   0.4321472685365301,    0.4321472685365301),
        Grad4::new( 0.8586508742123365,    0.508629699630796,    -0.044802370851755174, -0.044802370851755174),
        Grad4::new( 0.7821684431180708,    0.4321472685365301,   -0.12128480194602098,   0.4321472685365301),
        Grad4::new( 0.7821684431180708,    0.4321472685365301,    0.4321472685365301,   -0.12128480194602098),
        Grad4::new( 0.753341017856078,     0.37968289875261624,   0.37968289875261624,   0.37968289875261624)
    ];
    for i in 0 .. grad4.len() {
        grad4[i as usize].dx /= N4;
        grad4[i as usize].dy /= N4;
        grad4[i as usize].dz /= N4;
        grad4[i as usize].dw /= N4;
    }
    for i in 0 .. PSIZE {
        GRADIENTS_4D[i as usize] = grad4[i as usize % grad4.len()];
    }
}

static DATA_INIT: std::sync::Once = std::sync::Once::new();

struct StaticData {
    gradients_2d: &'static Vec<Grad2>,
    gradients_3d: &'static Vec<Grad3>,
    gradients_4d: &'static Vec<Grad4>,
    lookup_2d: &'static Vec<LatticePoint2D>,
    lookup_3d: &'static Vec<LatticePoint3D>,
    vertices_4d: &'static Vec<LatticePoint4D>,
}

fn get_static_data() -> StaticData {
    unsafe {
        DATA_INIT.call_once(|| {
            init_lattice_points();
            init_gradients();
        });
        StaticData {
            gradients_2d: &GRADIENTS_2D,
            gradients_3d: &GRADIENTS_3D,
            gradients_4d: &GRADIENTS_4D,
            lookup_2d: &LOOKUP_2D,
            lookup_3d: &LOOKUP_3D,
            vertices_4d: &VERTICES_4D,
        }
    }
}