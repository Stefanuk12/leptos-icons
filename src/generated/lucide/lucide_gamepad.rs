use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x2 = "10" x1 = "6" y2 = "12" ></ line > < line x2 = "8" x1 = "8" y2 = "14" y1 = "10" ></ line > < line x1 = "15" x2 = "15.01" y2 = "13" y1 = "13" ></ line > < line y1 = "11" x2 = "18.01" x1 = "18" y2 = "11" ></ line > < rect height = "12" y = "6" width = "20" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2")] } ;