use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" y1 = "12" x2 = "10" x1 = "6" ></ line > < line x2 = "8" x1 = "8" y1 = "10" y2 = "14" ></ line > < line y1 = "13" x1 = "15" x2 = "15.01" y2 = "13" ></ line > < line y1 = "11" x1 = "18" y2 = "11" x2 = "18.01" ></ line > < rect height = "12" y = "6" rx = "2" width = "20" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;