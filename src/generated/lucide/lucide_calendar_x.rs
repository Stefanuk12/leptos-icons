use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 2v4" ></ path > < path d = "M16 2v4" ></ path > < rect y = "4" x = "3" width = "18" height = "18" rx = "2" ></ rect > < path d = "M3 10h18" ></ path > < path d = "m14 14-4 4" ></ path > < path d = "m10 14 4 4" ></ path > < / > } } pub const LUCIDE_CALENDAR_X : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;