use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8V4H8" ></ path > < rect height = "12" rx = "2" width = "16" y = "8" x = "4" ></ rect > < path d = "M2 14h2" ></ path > < path d = "M20 14h2" ></ path > < path d = "M15 13v2" ></ path > < path d = "M9 13v2" ></ path > < / > } } pub const LUCIDE_BOT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;