use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect y = "4" height = "18" width = "18" rx = "2" x = "3" ></ rect > < path d = "M3 10h18" ></ path > < path d = "m14 14-4 4" ></ path > < path d = "m10 14 4 4" ></ path > < / > } } pub const LUCIDE_CALENDAR_X : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;