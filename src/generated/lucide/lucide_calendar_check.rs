use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect rx = "2" y = "4" height = "18" x = "3" width = "18" ></ rect > < path d = "M3 10h18" ></ path > < path d = "m9 16 2 2 4-4" ></ path > < / > } } pub const LUCIDE_CALENDAR_CHECK : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;