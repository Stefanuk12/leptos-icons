use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < circle r = "3" cx = "6" cy = "18" ></ circle > < circle cx = "18" cy = "16" r = "3" ></ circle > < / > } } pub const LUCIDE_MUSIC : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;