use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 11h.01" ></ path > < path d = "M14 6h.01" ></ path > < path d = "M18 6h.01" ></ path > < path d = "M6.5 13.1h.01" ></ path > < path d = "M22 5c0 9-4 12-6 12s-6-3-6-12c0-2 2-3 6-3s6 1 6 3" ></ path > < path d = "M17.4 9.9c-.8.8-2 .8-2.8 0" ></ path > < path d = "M10.1 7.1C9 7.2 7.7 7.7 6 8.6c-3.5 2-4.7 3.9-3.7 5.6 4.5 7.8 9.5 8.4 11.2 7.4.9-.5 1.9-2.1 1.9-4.7" ></ path > < path d = "M9.1 16.5c.3-1.1 1.4-1.7 2.4-1.4" ></ path > < / > } } pub const LUCIDE_DRAMA : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;