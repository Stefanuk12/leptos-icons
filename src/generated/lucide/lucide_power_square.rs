use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" width = "18" x = "3" y = "3" ></ rect > < path d = "M12 7v5" ></ path > < path d = "M8 9a5.14 5.14 0 0 0 4 8 4.95 4.95 0 0 0 4-8" ></ path > < / > } } pub const LucidePowerSquare : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;