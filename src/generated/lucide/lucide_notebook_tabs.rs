use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 6h4" ></ path > < path d = "M2 10h4" ></ path > < path d = "M2 14h4" ></ path > < path d = "M2 18h4" ></ path > < rect y = "2" rx = "2" width = "16" x = "4" height = "20" ></ rect > < path d = "M15 2v20" ></ path > < path d = "M15 7h5" ></ path > < path d = "M15 12h5" ></ path > < path d = "M15 17h5" ></ path > < / > } } pub const LUCIDE_NOTEBOOK_TABS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;