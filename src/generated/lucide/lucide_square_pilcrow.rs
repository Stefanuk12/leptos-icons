use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" x = "3" width = "18" height = "18" ></ rect > < path d = "M12 12H9.5a2.5 2.5 0 0 1 0-5H17" ></ path > < path d = "M12 7v10" ></ path > < path d = "M16 7v10" ></ path > < / > } } pub const LUCIDE_SQUARE_PILCROW : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;