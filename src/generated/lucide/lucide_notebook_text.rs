use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 6h4" ></ path > < path d = "M2 10h4" ></ path > < path d = "M2 14h4" ></ path > < path d = "M2 18h4" ></ path > < rect height = "20" rx = "2" width = "16" y = "2" x = "4" ></ rect > < path d = "M9.5 8h5" ></ path > < path d = "M9.5 12H16" ></ path > < path d = "M9.5 16H14" ></ path > < / > } } pub const LUCIDE_NOTEBOOK_TEXT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;