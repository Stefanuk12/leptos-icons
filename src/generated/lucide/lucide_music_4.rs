use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < path d = "m9 9 12-2" ></ path > < circle r = "3" cy = "18" cx = "6" ></ circle > < circle r = "3" cx = "18" cy = "16" ></ circle > < / > } } pub const LUCIDE_MUSIC_4 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;