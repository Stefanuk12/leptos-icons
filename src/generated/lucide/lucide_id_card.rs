use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 10h2" ></ path > < path d = "M16 14h2" ></ path > < path d = "M6.17 15a3 3 0 0 1 5.66 0" ></ path > < circle cx = "9" cy = "11" r = "2" ></ circle > < rect x = "2" rx = "2" width = "20" y = "5" height = "14" ></ rect > < / > } } pub const LUCIDE_ID_CARD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;