use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 22H4a2 2 0 0 1-2-2V6" ></ path > < path d = "m22 13-1.296-1.296a2.41 2.41 0 0 0-3.408 0L11 18" ></ path > < circle r = "2" cy = "8" cx = "12" ></ circle > < rect height = "16" x = "6" y = "2" rx = "2" width = "16" ></ rect > < / > } } pub const LUCIDE_IMAGES : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;