use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3v6" ></ path > < circle cy = "12" r = "3" cx = "12" ></ circle > < path d = "M12 15v6" ></ path > < / > } } pub const LucideGitCommitVertical : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;