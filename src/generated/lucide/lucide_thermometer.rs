use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" ></ path > < / > } } pub const LUCIDE_THERMOMETER : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;