use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6c0 2-2 2-2 4v10a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4V2h12z" ></ path > < line y1 = "6" y2 = "6" x2 = "18" x1 = "6" ></ line > < line y2 = "12" x2 = "12" y1 = "12" x1 = "12" ></ line > < / > } } pub const LUCIDE_FLASHLIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;