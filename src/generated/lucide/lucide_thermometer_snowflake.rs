use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12h10" ></ path > < path d = "M9 4v16" ></ path > < path d = "m3 9 3 3-3 3" ></ path > < path d = "M12 6 9 9 6 6" ></ path > < path d = "m6 18 3-3 1.5 1.5" ></ path > < path d = "M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" ></ path > < / > } } pub const LucideThermometerSnowflake : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24")] } ;