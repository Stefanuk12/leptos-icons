use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < path d = "m9 9 12-2" ></ path > < circle cx = "6" cy = "18" r = "3" ></ circle > < circle cx = "18" cy = "16" r = "3" ></ circle > < / > } } pub const LUCIDE_MUSIC_4 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;