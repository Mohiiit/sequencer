use std::collections::HashMap;

use cairo_vm::types::relocatable::Relocatable;
use cairo_vm::vm::vm_core::VirtualMachine;
use starknet_api::core::ContractAddress;

#[derive(Copy, Clone)]
pub struct StoragePtr(pub Relocatable);

#[derive(Copy, Clone)]
pub struct StateEntryPtr(pub Relocatable);

/// An equivalent to the `StateUpdatePointers` class in Python.
// TODO(Nimrod): Remove all `#[allow(dead_code)]` attributes after the code is fully implemented.
#[allow(dead_code)]
pub struct StateUpdatePointers {
    state_entries_ptr: Relocatable,
    classes_ptr: Relocatable,
    contract_address_to_state_entry_and_storage_ptr:
        HashMap<ContractAddress, (StateEntryPtr, StoragePtr)>,
}

impl StateUpdatePointers {
    #[allow(dead_code)]
    pub fn new(vm: &mut VirtualMachine) -> Self {
        Self {
            state_entries_ptr: vm.add_memory_segment(),
            classes_ptr: vm.add_memory_segment(),
            contract_address_to_state_entry_and_storage_ptr: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_contract_state_entry_and_storage_ptr(
        &mut self,
        contract_address: ContractAddress,
        vm: &mut VirtualMachine,
    ) -> (StateEntryPtr, StoragePtr) {
        *self.contract_address_to_state_entry_and_storage_ptr.entry(contract_address).or_insert((
            StateEntryPtr(vm.add_memory_segment()),
            StoragePtr(vm.add_memory_segment()),
        ))
    }

    #[allow(dead_code)]
    pub fn set_contract_state_entry_and_storage_ptr(
        &mut self,
        contract_address: ContractAddress,
        state_entry_ptr: StateEntryPtr,
        storage_ptr: StoragePtr,
    ) {
        self.contract_address_to_state_entry_and_storage_ptr
            .insert(contract_address, (state_entry_ptr, storage_ptr));
    }

    #[allow(dead_code)]
    pub fn get_classes_ptr(&self) -> Relocatable {
        self.classes_ptr
    }

    #[allow(dead_code)]
    pub fn set_classes_ptr(&mut self, ptr: Relocatable) {
        self.classes_ptr = ptr;
    }

    #[allow(dead_code)]
    pub fn get_state_entries_ptr(&self) -> Relocatable {
        self.state_entries_ptr
    }

    #[allow(dead_code)]
    pub fn set_state_entries_ptr(&mut self, ptr: Relocatable) {
        self.state_entries_ptr = ptr;
    }
}

pub fn get_contract_state_entry_and_storage_ptr(
    optional_state_update_pointers: &mut Option<StateUpdatePointers>,
    vm: &mut VirtualMachine,
    contract_address: ContractAddress,
) -> (StateEntryPtr, StoragePtr) {
    match optional_state_update_pointers {
        Some(state_update_pointers) => {
            state_update_pointers.get_contract_state_entry_and_storage_ptr(contract_address, vm)
        }
        None => (StateEntryPtr(vm.add_memory_segment()), StoragePtr(vm.add_memory_segment())),
    }
}
