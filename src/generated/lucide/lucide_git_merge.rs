use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" cx = "18" r = "3" ></ circle > < circle cy = "6" cx = "6" r = "3" ></ circle > < path d = "M6 21V9a9 9 0 0 0 9 9" ></ path > < / > } } pub const LUCIDE_GIT_MERGE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;