use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 6h8" ></ path > < path d = "M12 16h6" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < path d = "M8 11h7" ></ path > < / > } } pub const LUCIDE_CHART_GANTT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;