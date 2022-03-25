void PluginInit()
{
	g_Module.ScriptInfo.SetAuthor("Yoplitein");
    g_Module.ScriptInfo.SetContactInfo("yoplitein@gmail.com");
}

void dbgDrawLine(const Vector&in pt1, const Vector&in pt2, uint16 lifetimeDs = 25, uint8 r = 255, uint8 g = 127, uint8 b = 0)
{
	NetworkMessage msg(MSG_BROADCAST, NetworkMessages::SVC_TEMPENTITY, null);
	msg.WriteByte(TE_LINE);
	msg.WriteVector(pt1);
	msg.WriteVector(pt2);
	msg.WriteShort(lifetimeDs);
	msg.WriteByte(r);
	msg.WriteByte(g);
	msg.WriteByte(b);
	msg.End();
}

void commandTexTrace(const CCommand@ cmd)
{
	CBasePlayer@ ply = g_ConCommandSystem.GetCurrentPlayer();
	
	Vector forward, dummy, angles = ply.pev.angles;
	angles.x *= -Math.PI; // I have no idea why this is necessary for only pitch, yaw is fine as is
	g_EngineFuncs.AngleVectors(angles, forward, dummy, dummy);
	
	Vector origin = ply.EyePosition();
	TraceResult trace;
	g_Utility.TraceLine(origin, origin + forward * WORLD_BOUNDARY, ignore_monsters, dont_ignore_glass, ply.edict(), trace);
	
	Vector ray = trace.vecEndPos - origin;
	float len = ray.Length();
	ray = (ray / len) * (len + 1); // TraceTexture needs a slight bias
	
	string buffer;
	snprintf(buffer, "trace hit `%1`\n", g_Utility.TraceTexture(null, origin, origin + ray));
	g_PlayerFuncs.ClientPrint(ply, HUD_PRINTTALK, buffer);
	dbgDrawLine(origin, origin + ray);
	
	if(trace.pHit !is null)
	{
		entvars_t@ ent = @trace.pHit.vars;
		snprintf(buffer, " => hit brush entity `%1` with targetname `%2` netname `%3` BSP model index `%4`\n",
			ent.classname,
			ent.targetname,
			ent.netname,
			ent.modelindex
		);
		g_PlayerFuncs.ClientPrint(ply, HUD_PRINTTALK, buffer);
	}
}

void commandSetpos(const CCommand@ cmd)
{
	CBasePlayer@ ply = g_ConCommandSystem.GetCurrentPlayer();
	
	if(cmd.ArgC() != 4)
		return;
	
	Vector newPos;
	newPos.x = atof(cmd.Arg(1));
	newPos.y = atof(cmd.Arg(2));
	newPos.z = atof(cmd.Arg(3));
	g_EntityFuncs.SetOrigin(ply, newPos);
	
	string buffer;
	snprintf(buffer, "ok new pos %1\n", newPos.ToString());
	g_PlayerFuncs.ClientPrint(ply, HUD_PRINTNOTIFY, buffer);
}

void commandNoclip(const CCommand@ cmd)
{
	CBasePlayer@ ply = g_ConCommandSystem.GetCurrentPlayer();
	
	bool hasNoclip = ply.pev.movetype == MOVETYPE_NOCLIP;
    if(hasNoclip)
        ply.pev.movetype = MOVETYPE_WALK;
    else
        ply.pev.movetype = MOVETYPE_NOCLIP;
    
    string buffer;
    snprintf(buffer, "noclip %2\n", !hasNoclip ? "on" : "off");
    g_PlayerFuncs.ClientPrint(ply, HUD_PRINTNOTIFY, buffer);
}

CClientCommand commandTexTraceObj("textrace", "", @commandTexTrace);
CClientCommand commandSetposObj("setpos", "", @commandSetpos);
CClientCommand commandSetposObj("noclip", "", @commandNoclip);
