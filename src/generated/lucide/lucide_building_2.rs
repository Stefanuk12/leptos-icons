use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z" ></ path > < path d = "M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2" ></ path > < path d = "M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2" ></ path > < path d = "M10 6h4" ></ path > < path d = "M10 10h4" ></ path > < path d = "M10 14h4" ></ path > < path d = "M10 18h4" ></ path > < / > } } pub const LUCIDE_BUILDING_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;