use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" ></ path > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M9 9.563C9 9.252 9.252 9 9.563 9h4.874c.311 0 .563.252.563.563v4.874c0 .311-.252.563-.563.563H9.564A.562.562 0 0 1 9 14.437V9.564Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_STOP_CIRCLE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("fill" , "none") , ("stroke-width" , "1.5") , ("stroke" , "currentColor")] } ;