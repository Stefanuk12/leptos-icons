use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < path d = "m9 9 12-2" ></ path > < circle cx = "6" r = "3" cy = "18" ></ circle > < circle cx = "18" cy = "16" r = "3" ></ circle > < / > } } pub const LUCIDE_MUSIC_4 : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;