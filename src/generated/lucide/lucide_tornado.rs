use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 4H3" ></ path > < path d = "M18 8H6" ></ path > < path d = "M19 12H9" ></ path > < path d = "M16 16h-6" ></ path > < path d = "M11 20H9" ></ path > < / > } } pub const LucideTornado : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;