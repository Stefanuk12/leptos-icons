use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 16v5" ></ path > < path d = "M16 14v7" ></ path > < path d = "M20 10v11" ></ path > < path d = "m22 3-8.646 8.646a.5.5 0 0 1-.708 0L9.354 8.354a.5.5 0 0 0-.707 0L2 15" ></ path > < path d = "M4 18v3" ></ path > < path d = "M8 14v7" ></ path > < / > } } pub const LUCIDE_CHART_NO_AXES_COMBINED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;