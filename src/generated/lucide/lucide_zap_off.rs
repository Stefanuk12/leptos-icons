use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.513 4.856 13.12 2.17a.5.5 0 0 1 .86.46l-1.377 4.317" ></ path > < path d = "M15.656 10H20a1 1 0 0 1 .78 1.63l-1.72 1.773" ></ path > < path d = "M16.273 16.273 10.88 21.83a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14H4a1 1 0 0 1-.78-1.63l4.507-4.643" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_ZAP_OFF : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round")] } ;