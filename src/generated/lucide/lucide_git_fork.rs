use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "12" cy = "18" ></ circle > < circle cy = "6" cx = "6" r = "3" ></ circle > < circle cy = "6" r = "3" cx = "18" ></ circle > < path d = "M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9" ></ path > < path d = "M12 12v3" ></ path > < / > } } pub const LUCIDE_GIT_FORK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;