use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8V4H8" ></ path > < rect y = "8" x = "4" width = "16" height = "12" rx = "2" ></ rect > < path d = "M2 14h2" ></ path > < path d = "M20 14h2" ></ path > < path d = "M15 13v2" ></ path > < path d = "M9 13v2" ></ path > < / > } } pub const LUCIDE_BOT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;