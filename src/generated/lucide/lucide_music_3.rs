use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "4" cy = "18" ></ circle > < path d = "M16 18V2" ></ path > < / > } } pub const LUCIDE_MUSIC_3 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;