use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y2 = "12" y1 = "12" x2 = "10" ></ line > < line y1 = "10" y2 = "14" x1 = "8" x2 = "8" ></ line > < line x1 = "15" x2 = "15.01" y1 = "13" y2 = "13" ></ line > < line y2 = "11" y1 = "11" x1 = "18" x2 = "18.01" ></ line > < rect height = "12" y = "6" width = "20" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;