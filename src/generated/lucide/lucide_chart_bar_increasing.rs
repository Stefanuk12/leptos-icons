use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < path d = "M7 11h8" ></ path > < path d = "M7 16h12" ></ path > < path d = "M7 6h3" ></ path > < / > } } pub const LUCIDE_CHART_BAR_INCREASING : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;