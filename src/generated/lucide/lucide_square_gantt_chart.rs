use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" width = "18" height = "18" rx = "2" ></ rect > < path d = "M9 8h7" ></ path > < path d = "M8 12h6" ></ path > < path d = "M11 16h5" ></ path > < / > } } pub const LUCIDE_SQUARE_GANTT_CHART : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;