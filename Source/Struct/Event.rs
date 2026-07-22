#[derive(Debug, Clone)]
pub enum Struct {
	JobStarted {
		Directory:　String,
		Total:　usize,
	},
	Line {
		Directory:　String,
		Text:　String,
		IsStderr:　bool,
	},
	JobProgress {
		Directory:　String,
		Done:　usize,
		Total:　usize,
		Success:　bool,
	},
	JobFinished { Directory:　String, Success:　bool },
	IndexLockTimeout { Directory:　String },
	AllDone,
}
