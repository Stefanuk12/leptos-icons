use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "18" cx = "18" ></ circle > < circle cy = "6" r = "3" cx = "6" ></ circle > < path d = "M6 21V9a9 9 0 0 0 9 9" ></ path > < / > } } pub const LUCIDE_GIT_MERGE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;