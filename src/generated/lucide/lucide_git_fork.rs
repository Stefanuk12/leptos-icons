use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "18" cx = "12" ></ circle > < circle r = "3" cx = "6" cy = "6" ></ circle > < circle cx = "18" r = "3" cy = "6" ></ circle > < path d = "M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9" ></ path > < path d = "M12 12v3" ></ path > < / > } } pub const LUCIDE_GIT_FORK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;