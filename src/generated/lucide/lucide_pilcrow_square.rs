use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" width = "18" height = "18" y = "3" ></ rect > < path d = "M12 12H9.5a2.5 2.5 0 0 1 0-5H17" ></ path > < path d = "M12 7v10" ></ path > < path d = "M16 7v10" ></ path > < / > } } pub const LUCIDE_PILCROW_SQUARE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24")] } ;