use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 12a10.06 10.06 1 0 0-20 0Z" ></ path > < path d = "M12 12v8a2 2 0 0 0 4 0" ></ path > < path d = "M12 2v1" ></ path > < / > } } pub const LucideUmbrella : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;