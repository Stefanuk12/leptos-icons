use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "10" y2 = "12" y1 = "12" ></ line > < line x2 = "8" y2 = "14" x1 = "8" y1 = "10" ></ line > < line x1 = "15" x2 = "15.01" y2 = "13" y1 = "13" ></ line > < line x2 = "18.01" y1 = "11" x1 = "18" y2 = "11" ></ line > < rect x = "2" height = "12" y = "6" width = "20" rx = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24")] } ;