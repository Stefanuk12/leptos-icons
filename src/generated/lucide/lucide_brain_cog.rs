use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 5a3 3 0 1 0-5.997.142 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588 4 4 0 0 0 7.636 2.106 3.2 3.2 0 0 0 .164-.546c.028-.13.306-.13.335 0a3.2 3.2 0 0 0 .163.546 4 4 0 0 0 7.636-2.106 4 4 0 0 0 .556-6.588 4 4 0 0 0-2.526-5.77A3 3 0 1 0 12 5" ></ path > < path d = "M17.599 6.5a3 3 0 0 0 .399-1.375" ></ path > < path d = "M6.003 5.125A3 3 0 0 0 6.401 6.5" ></ path > < path d = "M3.477 10.896a4 4 0 0 1 .585-.396" ></ path > < path d = "M19.938 10.5a4 4 0 0 1 .585.396" ></ path > < path d = "M6 18a4 4 0 0 1-1.967-.516" ></ path > < path d = "M19.967 17.484A4 4 0 0 1 18 18" ></ path > < circle cx = "12" cy = "12" r = "3" ></ circle > < path d = "m15.7 10.4-.9.4" ></ path > < path d = "m9.2 13.2-.9.4" ></ path > < path d = "m13.6 15.7-.4-.9" ></ path > < path d = "m10.8 9.2-.4-.9" ></ path > < path d = "m15.7 13.5-.9-.4" ></ path > < path d = "m9.2 10.9-.9-.4" ></ path > < path d = "m10.5 15.7.4-.9" ></ path > < path d = "m13.1 9.2.4-.9" ></ path > < / > } } pub const LUCIDE_BRAIN_COG : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;