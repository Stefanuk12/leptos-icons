use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "4" cx = "12" cy = "18" ></ circle > < path d = "M16 18V2" ></ path > < / > } } pub const LUCIDE_MUSIC_3 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24")] } ;