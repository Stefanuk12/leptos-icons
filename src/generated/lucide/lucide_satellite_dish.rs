use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 10a7.31 7.31 0 0 0 10 10Z" ></ path > < path d = "m9 15 3-3" ></ path > < path d = "M17 13a6 6 0 0 0-6-6" ></ path > < path d = "M21 13A10 10 0 0 0 11 3" ></ path > < / > } } pub const LucideSatelliteDish : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;