use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 10h2" ></ path > < path d = "M16 14h2" ></ path > < path d = "M6.17 15a3 3 0 0 1 5.66 0" ></ path > < circle r = "2" cx = "9" cy = "11" ></ circle > < rect x = "2" height = "14" rx = "2" y = "5" width = "20" ></ rect > < / > } } pub const LUCIDE_ID_CARD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;