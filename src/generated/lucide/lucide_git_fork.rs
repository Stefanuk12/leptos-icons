use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" cx = "12" r = "3" ></ circle > < circle cx = "6" cy = "6" r = "3" ></ circle > < circle cy = "6" r = "3" cx = "18" ></ circle > < path d = "M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9" ></ path > < path d = "M12 12v3" ></ path > < / > } } pub const LucideGitFork : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;