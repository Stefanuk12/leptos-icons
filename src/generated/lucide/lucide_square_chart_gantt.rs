use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" y = "3" rx = "2" height = "18" ></ rect > < path d = "M9 8h7" ></ path > < path d = "M8 12h6" ></ path > < path d = "M11 16h5" ></ path > < / > } } pub const LUCIDE_SQUARE_CHART_GANTT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24")] } ;