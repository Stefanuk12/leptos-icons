use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v4" ></ path > < path d = "m6.8 15-3.5 2" ></ path > < path d = "m20.7 7-3.5 2" ></ path > < path d = "M6.8 9 3.3 7" ></ path > < path d = "m20.7 17-3.5-2" ></ path > < path d = "m9 22 3-8 3 8" ></ path > < path d = "M8 22h8" ></ path > < path d = "M18 18.7a9 9 0 1 0-12 0" ></ path > < / > } } pub const LucideFerrisWheel : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;