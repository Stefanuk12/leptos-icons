use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 8h1a4 4 0 1 1 0 8h-1" ></ path > < path d = "M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z" ></ path > < line x2 = "6" x1 = "6" y1 = "2" y2 = "4" ></ line > < line y1 = "2" x1 = "10" x2 = "10" y2 = "4" ></ line > < line y2 = "4" x2 = "14" y1 = "2" x1 = "14" ></ line > < / > } } pub const LucideCoffee : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;