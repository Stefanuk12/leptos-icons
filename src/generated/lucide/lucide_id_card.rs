use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 10h2" ></ path > < path d = "M16 14h2" ></ path > < path d = "M6.17 15a3 3 0 0 1 5.66 0" ></ path > < circle r = "2" cx = "9" cy = "11" ></ circle > < rect rx = "2" width = "20" height = "14" x = "2" y = "5" ></ rect > < / > } } pub const LUCIDE_ID_CARD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;