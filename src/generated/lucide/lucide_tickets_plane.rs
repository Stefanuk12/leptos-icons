use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.5 17h1.227a2 2 0 0 0 1.345-.52L18 12" ></ path > < path d = "m12 13.5 3.75.5" ></ path > < path d = "m4.5 8 10.58-5.06a1 1 0 0 1 1.342.488L18.5 8" ></ path > < path d = "M6 10V8" ></ path > < path d = "M6 14v1" ></ path > < path d = "M6 19v2" ></ path > < rect y = "8" width = "20" x = "2" height = "13" rx = "2" ></ rect > < / > } } pub const LUCIDE_TICKETS_PLANE : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;