use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "4" cy = "18" ></ circle > < path d = "M16 18V2" ></ path > < / > } } pub const LUCIDE_MUSIC_3 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;