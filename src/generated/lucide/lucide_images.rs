use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 22H4a2 2 0 0 1-2-2V6" ></ path > < path d = "m22 13-1.296-1.296a2.41 2.41 0 0 0-3.408 0L11 18" ></ path > < circle r = "2" cx = "12" cy = "8" ></ circle > < rect y = "2" x = "6" rx = "2" height = "16" width = "16" ></ rect > < / > } } pub const LUCIDE_IMAGES : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;