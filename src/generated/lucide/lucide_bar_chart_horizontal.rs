use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < path d = "M7 16h8" ></ path > < path d = "M7 11h12" ></ path > < path d = "M7 6h3" ></ path > < / > } } pub const LUCIDE_BAR_CHART_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;