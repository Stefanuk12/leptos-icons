use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 6H3" ></ path > < path d = "M10 12H3" ></ path > < path d = "M10 18H3" ></ path > < circle cx = "17" cy = "15" r = "3" ></ circle > < path d = "m21 19-1.9-1.9" ></ path > < / > } } pub const LucideTextSearch : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;