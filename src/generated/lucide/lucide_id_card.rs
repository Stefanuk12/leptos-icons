use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 10h2" ></ path > < path d = "M16 14h2" ></ path > < path d = "M6.17 15a3 3 0 0 1 5.66 0" ></ path > < circle r = "2" cy = "11" cx = "9" ></ circle > < rect rx = "2" x = "2" height = "14" y = "5" width = "20" ></ rect > < / > } } pub const LUCIDE_ID_CARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;