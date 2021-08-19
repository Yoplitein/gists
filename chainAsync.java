/**
	Sequentially schedules a series of CompletableFutures on the given executor.
	i.e. earlier futures must complete before subsequent futures are scheduled
*/
static CompletableFuture<Void> chainAsync(Stream<CompletableFuture<?>> tasks, Executor pool)
{
	chainAsync(tasks, pool, 1);
}

// ditto, but up to `parallel` futures are scheduled concurrently.
static CompletableFuture<Void> chainAsync(Stream<CompletableFuture<?>> tasks, Executor pool, int parallel)
{
	final var done = new CompletableFuture<Void>();
	final var empty = CompletableFuture.completedFuture(null);
	final var iter = tasks.iterator();
	
	final var nextFutures = new CompletableFuture[parallel];
	final var scheduleNext = new Cell<Runnable>(null);
	scheduleNext.val = () -> {
		try
		{
			if(!iter.hasNext())
				done.complete(null);
			else
			{
				Arrays.fill(nextFutures, empty);
				for(int i = 0; i < parallel; i++)
				{
					if(!iter.hasNext()) break;
					
					try { nextFutures[i] = iter.next(); }
					catch(Throwable err)
					{
						done.completeExceptionally(err);
						for(var task: nextFutures) task.completeExceptionally(err);
						return;
					}
				}
				
				final var next = CompletableFuture.allOf(nextFutures);
				next.thenRunAsync(scheduleNext.val, pool);
				next.exceptionallyAsync(err -> { done.completeExceptionally(err); return null; }, pool);
			}
		}
		catch(Throwable err)
		{
			done.completeExceptionally(err);
		}
	};
	scheduleNext.val.run();
	
	return done;
}